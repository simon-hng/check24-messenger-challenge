export type Chat = {
	id: number;
	name: string;
	last_message: string;
	updated_at: Date;
};

export function load() {
	const chats: Chat[] = [
		{
			id: 1,
			name: 'Reyees Herzog',
			last_message: 'Yes. You can create your own remix',
			updated_at: new Date('2011-10-10T14:48:00')
		},
		{
			id: 2,
			name: 'Colton Ruecker',
			last_message: 'lorem ipsum dolor sit amet',
			updated_at: new Date('2011-10-10T14:48:00')
		}
	];

	return {
		chats
	};
}
