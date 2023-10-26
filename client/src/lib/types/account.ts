export interface Account {
	id: string; // UUID
	name: string;
	picture: string;
	account_type: 'Customer' | 'ServiceProvider';
}
