import { Component } from '@angular/core';
import { CreateGame } from '../../models/game.model';
import { FormBuilder, FormGroup } from '@angular/forms';
import { GameService } from '../../services/game.service';
import { Router } from '@angular/router';
import { ImageService } from '../../services/image.service';
import { VideoService } from '../../services/video.service';

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

  genres: string[] = ['Action', 'Adventure', 'RPG', 'Strategy', 'Simulation', 'Sports', 'FPS', 'Platformer'];
  form: FormGroup;
  selectedFiles: File[] = [];
  selectedVideo: File | null = null;
  isSubmitting: boolean = false;
  imagePreviews: string[] = [];
  videoPreview: string | null = null;

  constructor(
    private fb: FormBuilder,
    private gameService: GameService,
    private imageService: ImageService,
    private videoService: VideoService,
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
      this.imagePreviews = [];

      this.selectedFiles.forEach(file => {
        const reader = new FileReader();
        reader.onload = (e: any) => {
          this.imagePreviews.push(e.target.result);
        };
        reader.readAsDataURL(file);
      });
    }
  }

  onVideoChange(event: any) {
    if (event.target.files && event.target.files.length > 0) {
      this.selectedVideo = event.target.files[0];

      if (this.selectedVideo) {
        this.videoPreview = URL.createObjectURL(this.selectedVideo);
      }
    }
  }

  ngOnDestroy() {
    if (this.videoPreview) {
      URL.revokeObjectURL(this.videoPreview);
    }
  }

  submit() {
    if (this.form.invalid) return;
    this.isSubmitting = true;

    this.gameService.createGame(this.form.value).subscribe({
      next: (game) => {
        const gameId = game.id;

        const uploadImages$ = this.selectedFiles.length > 0
          ? this.imageService.uploadImagesForGame(gameId, this.selectedFiles)
          : null;

        const uploadVideo$ = this.selectedVideo
          ? this.videoService.uploadVideosForGame(gameId, [this.selectedVideo])
          : null;

        // handle combinations cleanly
        if (uploadImages$) {
          uploadImages$.subscribe({
            error: (err) => console.error(err)
          });
        }

        if (uploadVideo$) {
          uploadVideo$.subscribe({
            error: (err) => console.error(err)
          });
        }

        setTimeout(() => {
          this.isSubmitting = false;
          this.router.navigate(['/games', gameId]);
        }, 500);
      },
      error: (err) => {
        console.error('Error creating game', err);
        this.isSubmitting = false;
      }
    });
  }
}
