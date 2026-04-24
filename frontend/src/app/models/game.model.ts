export interface Game {
  id: string;
  name: string;
  genre: string;
  description: string;
  release_date: string;
}

export interface CreateGame {
  name: string;
  genre: string;
  description: string;
  release_date: string;
}

export interface CreateGameResponse {
  id: string;
}