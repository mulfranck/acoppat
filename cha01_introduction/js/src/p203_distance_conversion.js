/**
 * Problem 1.2.3 (Distance Coversion)
 */


(function main() {
    const inFeet = 3281;
    const inMiles = 0.6214;
    const inYards = 1093.6;
    const inNMiles = 0.5396;


    let kg = 0;

    console.log('-'.repeat(54));
    console.log('Kilometers\tFeet\tYards\tNautical Miles\tMiles');
    console.log('-'.repeat(54));
    while ( kg <= 4 ) {

        
        console.log(`  ${kg}\t${Math.floor(kg * inFeet)}\t${Math.floor(kg * inYards)}\t${Math.floor(kg * inNMiles)}\t${Math.floor(kg * inMiles)}`);

        kg++;
    }
})()