import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class VideoService {

  private apiUrl = 'http://localhost:8000/api/videos';

  constructor(private http: HttpClient) { }

  uploadVideosForGame(gameId: string, files: File[]) {
    const formData = new FormData();
    files.forEach(file => formData.append('files', file, file.name));
    return this.http.post(this.apiUrl + "/" + gameId, formData);
  }

  getVideos(gameId: string) {
    return this.http.get<string[]>(this.apiUrl + "/" + gameId);
  }
}
