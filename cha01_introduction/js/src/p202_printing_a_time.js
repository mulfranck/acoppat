/**
 *  */ 

const readline = require('readline');



async function ask(question) {
    let rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });
    
    return new Promise((resolve, reject) => {
        rl.question("Enter the date in secs: ", (ans) => {
            resolve(ans);
            rl.close();
        });
    });
};

(async function main() {
    const HOUR = 3600;
    const MINS = 60;

    let inHour = 0;
    let inMin = 0;

    console.log("Nigga enter at most 5digits :)");
    let timeInSec = await ask();

    // let timeInSec = await 3812;
    let originalTime = await timeInSec;
    
    let isLenOk =  timeInSec.trim().length <= 5 ? true : false;

    while (isLenOk) {
        if ( +timeInSec < MINS ) {
            break;
        } else if ( +timeInSec >= HOUR ) {
            inHour = Math.floor(timeInSec / HOUR);
            timeInSec = Math.floor(timeInSec % HOUR);
        } else if ( +timeInSec >= MINS && +timeInSec <= HOUR ) {
            inMin = Math.floor(timeInSec / MINS);
            timeInSec = Math.floor(timeInSec % MINS);
        }
    }
    if (isLenOk) console.log(`${originalTime} Seconds = ${inHour} hours ${inMin} minutes ${timeInSec} seconds.`)
})();