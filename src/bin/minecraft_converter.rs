use fastnbt::Value;
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::io::Read;

/// Blocks that become resources (R) in the arena map
const RESOURCE_BLOCKS: &[&str] = &[
    "minecraft:chest",
    "minecraft:trapped_chest",
    "minecraft:diamond_ore",
    "minecraft:deepslate_diamond_ore",
    "minecraft:gold_ore",
    "minecraft:deepslate_gold_ore",
    "minecraft:iron_ore",
    "minecraft:deepslate_iron_ore",
    "minecraft:emerald_ore",
    "minecraft:deepslate_emerald_ore",
    "minecraft:ancient_debris",
    "minecraft:nether_gold_ore",
    "minecraft:glowstone",
    "minecraft:sea_lantern",
];

/// Blocks that are walls (#) — impassable
const WALL_BLOCKS: &[&str] = &[
    "minecraft:stone",
    "minecraft:cobblestone",
    "minecraft:mossy_cobblestone",
    "minecraft:bricks",
    "minecraft:stone_bricks",
    "minecraft:mossy_stone_bricks",
    "minecraft:cracked_stone_bricks",
    "minecraft:obsidian",
    "minecraft:bedrock",
    "minecraft:oak_log",
    "minecraft:spruce_log",
    "minecraft:birch_log",
    "minecraft:jungle_log",
    "minecraft:acacia_log",
    "minecraft:dark_oak_log",
    "minecraft:nether_brick",
    "minecraft:end_stone",
    "minecraft:end_stone_bricks",
    "minecraft:quartz_block",
    "minecraft:purpur_block",
    "minecraft:sandstone",
    "minecraft:red_sandstone",
    "minecraft:terracotta",
    "minecraft:netherrack",
];

fn is_resource(block: &str) -> bool {
    RESOURCE_BLOCKS.iter().any(|r| block.starts_with(r))
}

fn is_wall(block: &str) -> bool {
    WALL_BLOCKS.iter().any(|w| block.starts_with(w))
}

fn parse_varint(data: &[u8], pos: &mut usize) -> i32 {
    let mut result = 0i32;
    let mut shift = 0;
    loop {
        if *pos >= data.len() {
            break;
        }
        let byte = data[*pos];
        *pos += 1;
        result |= ((byte & 0x7F) as i32) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
    }
    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: minecraft-converter <input.schem> [output.map] [--y-level <y>]");
        eprintln!("");
        eprintln!("Options:");
        eprintln!("  output.map     Output file (default: input.map)");
        eprintln!("  --y-level <y>  Y slice to use (default: auto-detect densest layer)");
        eprintln!("");
        eprintln!("Examples:");
        eprintln!("  minecraft-converter dungeon.schem dungeon.map");
        eprintln!("  minecraft-converter castle.schem castle.map --y-level 64");
        std::process::exit(1);
    }

    let input_path = &args[1];
    
    let output_path = if args.len() > 2 && !args[2].starts_with("--") {
        args[2].clone()
    } else {
        input_path.replace(".schem", ".map")
    };

    let y_override: Option<i32> = args.windows(2)
        .find(|w| w[0] == "--y-level")
        .and_then(|w| w[1].parse().ok());

    // Read and decompress
    println!("Reading {}...", input_path);
    let file = std::fs::File::open(input_path)
        .unwrap_or_else(|e| { eprintln!("Could not open {}: {}", input_path, e); std::process::exit(1); });
    
    let mut decoder = GzDecoder::new(file);
    let mut raw = Vec::new();
    decoder.read_to_end(&mut raw)
        .unwrap_or_else(|e| { eprintln!("Failed to decompress: {}", e); std::process::exit(1); });

    // Parse NBT
    let nbt: HashMap<String, Value> = fastnbt::from_bytes(&raw)
        .unwrap_or_else(|e| { eprintln!("Failed to parse NBT: {}", e); std::process::exit(1); });

    // Extract dimensions
    let width = match nbt.get("Width") {
        Some(Value::Short(v)) => *v as i32,
        _ => { eprintln!("Missing Width in schematic"); std::process::exit(1); }
    };
    let height = match nbt.get("Height") {
        Some(Value::Short(v)) => *v as i32,
        _ => { eprintln!("Missing Height in schematic"); std::process::exit(1); }
    };
    let length = match nbt.get("Length") {
        Some(Value::Short(v)) => *v as i32,
        _ => { eprintln!("Missing Length in schematic"); std::process::exit(1); }
    };

    println!("Schematic dimensions: {}x{}x{} (W x H x L)", width, height, length);

    // Extract palette
    let palette_nbt = match nbt.get("Palette") {
        Some(Value::Compound(p)) => p,
        _ => { eprintln!("Missing Palette in schematic"); std::process::exit(1); }
    };

    let mut palette: HashMap<i32, String> = HashMap::new();
    for (block_name, index_val) in palette_nbt.iter() {
        if let Value::Int(idx) = index_val {
            // Strip block state properties e.g. "minecraft:chest[facing=north]" -> "minecraft:chest"
            let clean_name = block_name.split('[').next().unwrap_or(block_name).to_string();
            palette.insert(*idx, clean_name);
        }
    }

    println!("Palette: {} block types", palette.len());

    // Extract block data
    let block_data = match nbt.get("BlockData") {
        Some(Value::ByteArray(d)) => d.iter().map(|b| *b as u8).collect::<Vec<u8>>(),
        _ => { eprintln!("Missing BlockData in schematic"); std::process::exit(1); }
    };

    // Decode varints into block indices
    let total_blocks = (width * height * length) as usize;
    let mut blocks: Vec<i32> = Vec::with_capacity(total_blocks);
    let mut pos = 0;
    while pos < block_data.len() && blocks.len() < total_blocks {
        blocks.push(parse_varint(&block_data, &mut pos));
    }

    println!("Decoded {} blocks", blocks.len());

    // Block index: y * (width * length) + z * width + x
    let get_block = |x: i32, y: i32, z: i32| -> &str {
        let idx = (y * width * length + z * width + x) as usize;
        if idx >= blocks.len() { return "minecraft:air"; }
        palette.get(&blocks[idx]).map(|s| s.as_str()).unwrap_or("minecraft:air")
    };

    // Find best Y level if not specified
    let y_level = if let Some(y) = y_override {
        y
    } else {
        // Auto-detect: find Y layer with most non-air, non-bedrock blocks
        let mut best_y = 0;
        let mut best_count = 0;
        for y in 0..height {
            let count = (0..length).flat_map(|z| (0..width).map(move |x| (x, z)))
                .filter(|(x, z)| {
                    let b = get_block(*x, y, *z);
                    b != "minecraft:air" && b != "minecraft:bedrock"
                })
                .count();
            if count > best_count {
                best_count = count;
                best_y = y;
            }
        }
        println!("Auto-detected Y level: {} ({} non-air blocks)", best_y, best_count);
        best_y
    };

    // Generate 2D map from Y slice
    let mut map_lines: Vec<String> = Vec::new();
    let mut resource_count = 0;
    let mut wall_count = 0;

    for z in 0..length {
        let mut line = String::new();
        for x in 0..width {
            let block = get_block(x, y_level, z);
            if is_resource(block) {
                line.push('R');
                resource_count += 1;
            } else if is_resource(block) {
                line.push('R');
            } else if block == "minecraft:air" {
                line.push('.');
            } else if is_wall(block) {
                line.push('.');  // walls as passable for now, future: '#'
                wall_count += 1;
            } else {
                line.push('.');
            }
        }
        map_lines.push(line);
    }

    // Write output
    let mut output = format!(
        "# Generated from {} by minecraft-converter\n         # Dimensions: {}x{} (X x Z), Y-level: {}\n         # Resources: {}, Walls: {}\n         # R = resource, . = passable\n",
        input_path, width, length, y_level, resource_count, wall_count
    );

    for line in &map_lines {
        output.push_str(line);
        output.push('\n');
    }

    std::fs::write(&output_path, &output)
        .unwrap_or_else(|e| { eprintln!("Failed to write {}: {}", output_path, e); std::process::exit(1); });

    println!("Written to {}", output_path);
    println!("  Dimensions: {}x{}", width, length);
    println!("  Resources:  {}", resource_count);
    println!("  Y-level:    {}", y_level);
    println!("");
    println!("Run the arena with this map:");
    println!("  SWARM_MAP_FILE={} cargo run --bin swarm-arena", output_path);
}
