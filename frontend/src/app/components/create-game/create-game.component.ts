import { Component } from '@angular/core';
import { CreateGame } from '../../models/game.model';
import { FormBuilder, FormGroup } from '@angular/forms';
import { GameService } from '../../services/game.service';
import { Router } from '@angular/router';
import { ImageService } from '../../services/image.service';

@Component({
  selector: 'app-create-game',
  standalone: false,
  templateUrl: './create-game.component.html',
  styleUrl: './create-game.component.css'
})
export class CreateGameComponent {
  game: CreateGame = {
    name: '',
    genre: '',
    description: '',
    release_date: ''
  };

  form: FormGroup;
  selectedFiles: File[] = [];
  isSubmitting: boolean = false;

  constructor(
    private fb: FormBuilder,
    private gameService: GameService,
    private imageService: ImageService,
    private router: Router
  ) {
    this.form = this.fb.group({
      name: [''],
      genre: [''],
      description: [''],
      release_date: ['']
    });
  }

  onFilesChange(event: any) {
    if (event.target.files && event.target.files.length > 0) {
      this.selectedFiles = Array.from(event.target.files);
    }
  }

  submit() {
    if (this.form.invalid) return;
    this.isSubmitting = true;

    this.gameService.createGame(this.form.value).subscribe({
      next: (game) => {
        console.log('Game created:', game);

        const gameId = game.id;

        if (this.selectedFiles.length > 0) {
          this.imageService.uploadImagesForGame(gameId, this.selectedFiles).subscribe({
            next: () => console.log('Images uploaded successfully'),
            error: (err) => console.error('Error uploading images', err)
          });
        } else {
          this.isSubmitting = false;
          this.router.navigate(['/games', gameId]);
        }
      },
      error: (err) => {
        console.error('Error creating game', err);
        this.isSubmitting = false;
      }
    });
  }
}
