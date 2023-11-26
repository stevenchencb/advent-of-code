import { getFileLines } from '../../utils';

const lines = await getFileLines('./input.txt');

let group = 1;
let elfA: string[] = [];
let elfB: string[] = [];
let elfC: string[] = [];

const priorities = lines.reduce<Record<number, number>>((acc, curr, i) => {
	if (i % 3 === 0) {
		elfA = [...new Set(curr)];
	} else if (i % 3 === 1) {
		elfB = [...new Set(curr)];
	} else {
		elfC = [...new Set(curr)];
		const appearsInAll = elfA.filter((i) => elfB.includes(i) && elfC.includes(i))[0];
		acc[group] = getPriority(appearsInAll);
		group++;
	}

	return acc;
}, {});

console.log(Object.values(priorities).reduce((acc, curr) => acc + curr, 0));

function getPriority(c: string) {
	const charCode = c.charCodeAt(0);

	return charCode >= 97 ? charCode - 96 : charCode - 38;
}
