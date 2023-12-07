import { extractMultiple, getFileInput } from '../utils';
import { HandTypeOrder, Card, CardOrder, HandType } from './part1-models';

const input = await getFileInput(7);

export async function solve() {
	const regex = /(?<hand>[2-9TJQKA]{5}) (?<bid>\d+)/g;

	const handsAndBids = extractMultiple(input, regex, ['hand', 'bid'] as const)
		.map((hb) => ({ ...hb, bid: Number.parseInt(hb.bid ?? 0) }))
		.filter((h): h is { hand: string; bid: number } => !!h.hand);

	handsAndBids.sort((h1, h2) => sortHandsAsc(h1.hand, h2.hand));

	return handsAndBids.reduce((acc, curr, i) => {
		acc += curr.bid * (i + 1);
		return acc;
	}, 0);
}

function sortHandsAsc(hand1: string, hand2: string): number {
	const hand1Type = determineHandType(hand1);
	const hand2Type = determineHandType(hand2);

	return HandTypeOrder[hand1Type] - HandTypeOrder[hand2Type] || compareCards(hand1, hand2);
}

function compareCards(hand1: string, hand2: string): number {
	const cardsOfHand1 = [...hand1.matchAll(/[2-9TJQKA]/g)].map((m) => m[0]) as Card[];
	const cardsOfHand2 = [...hand2.matchAll(/[2-9TJQKA]/g)].map((m) => m[0]) as Card[];

	for (let i = 0; i < 5; i++) {
		if (CardOrder[cardsOfHand1[i]] < CardOrder[cardsOfHand2[i]]) {
			return -1;
		} else if (CardOrder[cardsOfHand1[i]] > CardOrder[cardsOfHand2[i]]) {
			return 1;
		}
	}
	return 0;
}

function determineHandType(hand: string): HandType {
	const cards = [...hand.matchAll(/[2-9TJQKA]/g)].map((m) => m[0]);

	const numOfCards = cards.reduce<Record<string, number>>((acc, curr) => {
		acc[curr] = (acc[curr] ?? 0) + 1;
		return acc;
	}, {});

	const typeOfCards = Object.keys(numOfCards).length;
	const highestNumOfSameCard = Object.values(numOfCards).sort((a, b) => b - a)[0];
	const secondHighestNumOfSameCard = Object.values(numOfCards).sort((a, b) => b - a)[1];

	switch (typeOfCards) {
		case 1:
			return 'QUINTUPLET';
		case 2:
			if (highestNumOfSameCard === 4) {
				return 'QUADRUPLET';
			} else {
				return 'FULL_HOUSE';
			}
		case 3:
			if (secondHighestNumOfSameCard === 1) {
				return 'TRIPLET';
			} else {
				return 'TWO_PAIR';
			}
		case 4:
			return 'ONE_PAIR';
		case 5:
			return 'HIGH_CARD';
		default:
			return 'HIGH_CARD';
	}
}
