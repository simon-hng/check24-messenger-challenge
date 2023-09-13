import { faker } from '@faker-js/faker';
export type Chat = {
	id: number;
	avatar: string;
	name: string;
	last_message: string;
	updated_at: Date;
	count_unread: number;
};

export function load() {
	const chats: Chat[] = Array.from(Array(10).keys()).map((chatId) => ({
		id: chatId,
		avatar: faker.image.avatar(),
		name: faker.person.fullName(),
		last_message: faker.lorem.text(),
		updated_at: faker.date.recent(),
		count_unread: faker.number.int({ min: 0, max: 4 })
	}));

	return {
		chats
	};
}
