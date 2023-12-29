async function main() {
	await runPart1(20);
	await runPart2(20);
}

async function runPart1(day: number) {
	await import(`./Day ${day}/part1.ts`).then(async (part1) => {
		const start = performance.now();

		const result = await part1.solve();

		const end = performance.now();

		console.log(`\nSolution for Day ${day} Part 1: ${result}`);
		console.log(`Time elapsed: ${Math.round((end - start + Number.EPSILON) * 100) / 100}ms\n`);
	});
}

async function runPart2(day: number) {
	await import(`./Day ${day}/part2.ts`).then(async (part2) => {
		const start = performance.now();

		const result = await part2.solve();

		const end = performance.now();

		console.log(`Solution for Day ${day} Part 2: ${result}`);
		console.log(`Time elapsed: ${Math.round((end - start + Number.EPSILON) * 100) / 100}ms\n`);
	});
}

main();
