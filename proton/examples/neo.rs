/*
    Appellation: neo <module>
    Contrib: @FL03
*/
use proton::prelude::*;
use proton::topo::motion::MotionPlanner;
use rstm::State;

fn main() -> proton::Result {
    let ruleset = proton::ruleset! {
        (0, 0) => (Right, 0, 4),
        (0, 4) => (Right, 1, 7),
        (0, 7) => (Left, 0, 0),
        (1, 0) => (Left, 1, 7),
        (1, 4) => (Left, 0, 11),
        (1, 7) => (Right, 1, 4),
        (0, 11) => (Left, 1, 7),
        (1, 11) => (Right, 0, 0)
    };
    println!("Tonnetz Integration Example");
    println!("==========================\n");

    // Create the global Tonnetz structure
    let mut tonnetz = Tonnetz::new();

    // Add pitch classes (0-11)
    for i in 0..12 {
        tonnetz.add_pitch_class(i);
    }

    // Add some common triads to the Tonnetz
    let c_major = tonnetz.add_triad([0, 4, 7], TriadClass::Major)?; // C Major
    let a_minor = tonnetz.add_triad([9, 0, 4], TriadClass::Minor)?; // A Minor
    let e_minor = tonnetz.add_triad([4, 7, 11], TriadClass::Minor)?; // E Minor
    let g_major = tonnetz.add_triad([7, 11, 2], TriadClass::Major)?; // G Major

    // Compute transformations between triads
    tonnetz.compute_transformations();

    println!("Created Tonnetz with 4 triads:");
    println!("- C Major (0,4,7)");
    println!("- A Minor (9,0,4)");
    println!("- E Minor (4,7,11)");
    println!("- G Major (7,11,2)\n");

    // Create a UTM for our first plant
    let utm = WolframUTM::new([0, 4, 7], State(0))
        .with_ruleset(ruleset)
        .with_modulus(12);

    // Deploy the UTM to C Major triad
    tonnetz.deploy_utm(c_major, utm.clone())?;

    println!("Deployed UTM to C Major triad\n");

    println!("Deployed UTM to C Major triad");

    // Try to find path from C Major to G Major
    let planner = MotionPlanner::new(&tonnetz);

    // Analyze what triads are accessible from C Major
    println!("\nTransformations available from C Major:");
    let available = planner.analyze_transformations(c_major);
    for (transform, id, pitches) in available {
        println!("- {:?} → {:?} (Triad ID: {:?})", transform, pitches, id);
    }

    // Method 1: Find path between specific triads (may not always work)
    println!("\nFinding path from C Major to G Major...");
    match planner.find_path(c_major, g_major) {
        Some(path) => {
            println!("Path found! Transformations: {:?}", path.transforms);
            println!("Executing path...");
            planner.execute_path(&path)?;
        }
        None => println!("No direct path found between these triads"),
    }

    // Method 2: Find path to include specific pitch class
    println!("\nFinding path to include pitch class 2 (D)...");
    match planner.find_path_to_symbol(c_major, 2) {
        Some(path) => {
            println!("Path found! Transformations: {:?}", path.transforms);
            println!("Path visits these triads: {:?}", path.path);

            // Show the triads in the path
            for &triad_id in &path.path {
                if let Some(plant) = tonnetz.plants.get(&triad_id) {
                    println!("- {:?} {:?}", plant.class(), plant.alphabet());
                }
            }
        }
        None => println!("No path found to include pitch class 2"),
    }

    // Method 3: Suggest possible transformations to include a symbol
    println!("\nSuggesting transformations to include pitch class 2 (D)...");
    let suggestions = planner.suggest_transformations_for_symbol(c_major, 2);
    if suggestions.is_empty() {
        println!("No direct transformation available to include pitch class 2");
        println!("Calculating which triads need to be added to the Tonnetz...");

        // Try to calculate what triads would be needed
        let c_major_triad = tonnetz.plants.get(&c_major).unwrap();

        // Try each transformation and see what would result
        for transform in [
            Transformation::Leading,
            Transformation::Parallel,
            Transformation::Relative,
        ] {
            let triad = c_major_triad.apply_transform(transform);
            println!("- Applying {transform:?} would create: {triad:?}");
            if triad.contains(&2) {
                println!("  This would include pitch class 2 (D)!");
            }
        }
    } else {
        for (transform, triad_id) in suggestions {
            if let Some(triad) = tonnetz.plants.get(&triad_id) {
                println!("- Apply {transform:?} to reach {triad:?}",);
            }
        }
    }

    Ok(())
}
