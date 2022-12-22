const readline = require("readline");

async function prompt(readling) {
	const rl = readline.createInterface({
		input: process.stdin,
		output: process.stdout,
	});

	return new Promise((resolve, reject) => {
		rl.question(`Enter the ${readling} reading: `, ans => {
			resolve(ans)
			rl.close();
		});
	});
}


(async function main() {
	const fr = await prompt('first');
	const sr = await prompt('second');
	const tr = await prompt('third');

	let sum = +fr + +sr + +tr

	console.log("\nTHE THREE TIMES ARE", +fr, +sr, +tr);
	console.log("THE TOTAL COOLING TIME IS ", sum, "SECONDS")
})();
