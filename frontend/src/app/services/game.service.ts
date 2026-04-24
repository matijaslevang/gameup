import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { CreateGame, CreateGameResponse, Game } from '../models/game.model';

@Injectable({
  providedIn: 'root'
})
export class GameService {

  private apiUrl = 'http://localhost:8000/api/games';

  constructor(private http: HttpClient) {}

  getGames(search?: string, genre?: string): Observable<Game[]> {
    let params = new HttpParams();

    if (search && search.trim() !== '') {
      params = params.set('name', search);
    }

    if (genre && genre.trim() !== '') {
      params = params.set('genre', genre);
    }

    return this.http.get<Game[]>(this.apiUrl, { params });
  }

  createGame(data: CreateGame): Observable<CreateGameResponse> {
    return this.http.post<CreateGameResponse>(this.apiUrl, data);
  }

  getGame(id: string): Observable<Game> {
    return this.http.get<Game>(this.apiUrl + "/" + id)
  }

  deleteGame(id: string): Observable<any> {
    return this.http.delete<any>(this.apiUrl + "/" + id)
  }

  editGame(data: CreateGame, id: string): Observable<CreateGameResponse> {
    return this.http.put<CreateGameResponse>(this.apiUrl + "/" + id, data)
  }
}