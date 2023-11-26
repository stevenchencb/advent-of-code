import { getFileLines } from '../../utils';

const lines = await getFileLines('./input.txt');

const rounds = lines.map((x) => x.replace(/\s/g, ''));

// X = loss, Y = draw, Z = win
const scoreMap: Record<string, number> = {
	AX: 3,
	AY: 4,
	AZ: 8,
	BX: 1,
	BY: 5,
	BZ: 9,
	CX: 2,
	CY: 6,
	CZ: 7,
};

const score = rounds.reduce((acc, curr) => {
	acc += scoreMap[curr];
	return acc;
}, 0);

console.log(score);
