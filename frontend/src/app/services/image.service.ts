import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ImageService {

  private apiUrl = 'http://localhost:8000/api/images';

  constructor(private http: HttpClient) { }

  uploadImagesForGame(gameId: string, files: File[]) {
    const formData = new FormData();
    files.forEach(file => formData.append('files', file, file.name));
    for (let entry of formData.entries()) {
    }
    return this.http.post(this.apiUrl + "/" + gameId, formData);
  }

  getImages(gameId: string) {
    return this.http.get<string[]>(this.apiUrl + "/" + gameId);
  }

  deleteImagesForGame(gameId: string): any {
    return this.http.delete<any>(this.apiUrl + "/" + gameId)
  }
}
