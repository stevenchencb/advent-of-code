export async function getFileInput(filename: string): Promise<string> {
	return Bun.file(filename).text();
}

export async function getFileLines(filename: string): Promise<string[]> {
	return (await Bun.file(filename).text()).split('\n');
}
