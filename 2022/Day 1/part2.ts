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

const caloriesSorted = Object.values(elfCaloriesMap).sort((a, b) => b - a);

console.log(caloriesSorted[0] + caloriesSorted[1] + caloriesSorted[2]);
