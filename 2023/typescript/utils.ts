export async function getFileInput(filename: string): Promise<string> {
	return Bun.file(filename).text();
}

export async function getFileLines(filename: string): Promise<string[]> {
	return (await Bun.file(filename).text()).split('\n');
}

export function extractWithRegex(s: string, regexp: string | RegExp, groupToExtract: string | number, regexFlags = 'gi') {
	const regex = new RegExp(regexp, regexFlags);
	const matches = regex.exec(s);
	return (typeof groupToExtract === 'string' ? (matches?.groups ?? {})[groupToExtract] : matches?.at(groupToExtract)) ?? '';
}

export function intersection<T>(a: T[], b: T[]) {
	return a.filter((x) => !!b.find((y) => x === y));
}
