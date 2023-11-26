import { getFileLines } from '../../utils';

const lines = await getFileLines('./input.txt');

const prioritiesSum = lines.reduce((acc, curr) => {
	const compartmentA = [...new Set(curr.substring(0, curr.length / 2))];
	const compartmentB = [...new Set(curr.substring(curr.length / 2))];
	const appearsInBoth = compartmentA.filter((i) => compartmentB.includes(i))[0];
	acc += getPriority(appearsInBoth);
	return acc;
}, 0);

console.log(prioritiesSum);

function getPriority(c: string) {
	const charCode = c.charCodeAt(0);

	return charCode >= 97 ? charCode - 96 : charCode - 38;
}
