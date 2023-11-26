import { getFileLines } from '../../utils';

const lines = await getFileLines('./input.txt');
let elfNumber = 1;

const elfCaloriesMap = lines.reduce<Record<number, number>>((acc, curr) => {
	if (curr === '') {
		elfNumber++;
		return acc;
	}
	acc[elfNumber] = (acc[elfNumber] ?? 0) + Number.parseInt(curr);
	return acc;
}, {});

const calories = Object.values(elfCaloriesMap);

console.log(calories.sort((a, b) => b - a)[0]);
