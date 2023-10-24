export interface Conversation {
	id: string; // TODO UUID
	state: 'Accepted' | 'Quoted' | 'Rejected';
	created_at: Date;
}
