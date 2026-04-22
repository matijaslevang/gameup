import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ImageService {

  private apiUrl = 'http://localhost:8000/api/images';

  constructor(private http: HttpClient) { }

  uploadImagesForGame(gameId: string, files: File[]) {
    console.log(files)
    const formData = new FormData();
    files.forEach(file => formData.append('files', file, file.name));
    console.log('FormData keys:', Array.from(formData.keys()));
    for (let entry of formData.entries()) {
      console.log(entry[0], entry[1]);
    }
    return this.http.post(this.apiUrl + "/" + gameId, formData);
  }
}
