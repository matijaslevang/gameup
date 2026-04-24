import { Component, OnInit } from '@angular/core';
import { CreateGame } from '../../models/game.model';
import { FormBuilder, FormGroup } from '@angular/forms';
import { GameService } from '../../services/game.service';
import { ImageService } from '../../services/image.service';
import { VideoService } from '../../services/video.service';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'app-edit-game',
  standalone: false,
  templateUrl: './edit-game.component.html',
  styleUrl: './edit-game.component.css'
})
export class EditGameComponent implements OnInit {
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

  isImageChanged = false;
  isVideoChanged = false;

  constructor(
    private fb: FormBuilder,
    private gameService: GameService,
    private imageService: ImageService,
    private videoService: VideoService,
    private router: Router,
    private route: ActivatedRoute
  ) {
    this.form = this.fb.group({
      name: [''],
      genre: [''],
      description: [''],
      release_date: ['']
    });
  }

  ngOnInit(): void {
    const id = this.route.snapshot.paramMap.get('id');
    this.gameService.getGame(id!).subscribe({
      next: (game) => {
        this.game = game;

        this.form.patchValue({
          name: game.name,
          genre: game.genre,
          description: game.description,
          release_date: game.release_date
        });
      },
      error: (err) => console.error(err)
    });

    // load images
    this.imageService.getImages(id!).subscribe({
      next: (images) => {
        this.imagePreviews = images;
      },
      error: (err) => console.error(err)
    });

    // load videos
    this.videoService.getVideos(id!).subscribe({
      next: (videos) => {
        if (videos.length > 0) {
          this.videoPreview = videos[0]; // assuming one video
        }
      },
      error: (err) => console.error(err)
    });
  }

  onFilesChange(event: any) {
    if (event.target.files && event.target.files.length > 0) {
      this.isImageChanged = true;
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
      this.isVideoChanged = true;
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
    const id = this.route.snapshot.paramMap.get('id');

    if (this.form.invalid) return;
    this.isSubmitting = true;

    this.gameService.editGame(this.form.value, id!).subscribe({
      next: (game) => {

        if (this.isImageChanged) {
          this.imageService.deleteImagesForGame(id!).subscribe({
            next: () => {
              if (this.selectedFiles.length > 0) {
                this.imageService.uploadImagesForGame(id!, this.selectedFiles).subscribe();
              }
            }
          });
        }

        if (this.isVideoChanged) {
        this.videoService.deleteVideosForGame(id!).subscribe({
          next: () => {
            if (this.selectedVideo) {
              this.videoService.uploadVideosForGame(id!, [this.selectedVideo]).subscribe();
            }
          }
        });
      }

        setTimeout(() => {
          this.isSubmitting = false;
          this.router.navigate(['/games', id]);
        }, 500);
      },
      error: (err) => {
        console.error('Error creating game', err);
        this.isSubmitting = false;
      }
    });
  }
}
