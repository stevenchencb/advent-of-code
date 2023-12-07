import { extractOne, getFileInput } from '../utils';

const input = await getFileInput(5);

export async function solve() {
	const seedsAndMapsStrings = input.split('\n\n');

	const seedToSoilMap = getMap(seedsAndMapsStrings, 'seed-to-soil');
	const soilToFertMap = getMap(seedsAndMapsStrings, 'soil-to-fertilizer');
	const fertToWaterMap = getMap(seedsAndMapsStrings, 'fertilizer-to-water');
	const waterToLightMap = getMap(seedsAndMapsStrings, 'water-to-light');
	const lightToTempMap = getMap(seedsAndMapsStrings, 'light-to-temperature');
	const tempToHumidMap = getMap(seedsAndMapsStrings, 'temperature-to-humidity');
	const humidToLocationMap = getMap(seedsAndMapsStrings, 'humidity-to-location');

	const seeds = [...seedsAndMapsStrings[0].matchAll(/\d+/g)].map((m) => Number.parseInt(m[0]));

	const locations = seeds.map((n) =>
		map(
			map(map(map(map(map(map(n, seedToSoilMap), soilToFertMap), fertToWaterMap), waterToLightMap), lightToTempMap), tempToHumidMap),
			humidToLocationMap
		)
	);

	return locations.sort((a, b) => a - b)[0];
}

function getMap(seedsAndMapsStrings: string[], mapName: string) {
	const mapLines =
		seedsAndMapsStrings
			.find((s) => s.includes(mapName))
			?.split('\n')
			?.slice(1) ?? [];
	const resultMap: number[][] = [];

	for (const line of mapLines) {
		const regex = /(?<destStart>\d+) (?<srcStart>\d+) (?<range>\d+)/;
		const destStart = Number.parseInt(extractOne(line, regex, 'destStart'));
		const srcStart = Number.parseInt(extractOne(line, regex, 'srcStart'));
		const range = Number.parseInt(extractOne(line, regex, 'range'));

		resultMap.push([srcStart, destStart, range]);
	}

	return resultMap;
}

function map(n: number, map: number[][]): number {
	for (const srcDestRange of map) {
		if (srcDestRange[0] <= n && srcDestRange[0] + srcDestRange[2] >= n) {
			return srcDestRange[1] + (n - srcDestRange[0]);
		}
	}

	return n;
}
