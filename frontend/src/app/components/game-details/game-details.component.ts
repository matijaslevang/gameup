import { HttpClient } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { Game } from '../../models/game.model';
import { ImageService } from '../../services/image.service';
import { GameService } from '../../services/game.service';
import { VideoService } from '../../services/video.service';
import { AuthService } from '../../services/auth.service';

@Component({
  selector: 'app-game-details',
  standalone: false,
  templateUrl: './game-details.component.html',
  styleUrl: './game-details.component.css'
})
export class GameDetailsComponent implements OnInit {

  game: any;
  images: string[] = [];
  videos: string[] = [];
  currentImageIndex: number = 0;
  isLoggedIn: boolean = false;

  constructor(
    private route: ActivatedRoute,
    private imageService: ImageService,
    private gameService: GameService,
    private videoService: VideoService,
    private authService: AuthService,
    private router: Router
  ) {
    this.isLoggedIn = authService.getToken() !== null
  }

  ngOnInit() {
    const id = this.route.snapshot.paramMap.get('id');

    this.gameService.getGame(id!).subscribe({
      next: (game) => {
        this.game = game
        this.imageService.getImages(id!).subscribe({
          next: (images) => {
            this.images = images
            console.log(images)
          },
          error: (err) => {
            console.error('Error loading images', err);
          }
        })

        this.videoService.getVideos(id!).subscribe({
          next: (video) => {
            this.videos = video
            console.log(video)
          },
          error: (err) => {
            console.error('Error loading videos', err)
          }
        })
      }
    })
  }

  onEdit() {
    this.router.navigate(['/games/edit', this.game?.id]);
  }

  onDelete() {
    if (!confirm('Are you sure you want to delete this game?')) return;

    this.gameService.deleteGame(this.game?.id).subscribe({
      next: () => this.router.navigate(['']),
      error: (err) => console.error(err)
    });
  }

  nextImage() {
    if (this.images.length === 0) return;
    this.currentImageIndex = (this.currentImageIndex + 1) % this.images.length;
  }

  prevImage() {
    if (this.images.length === 0) return;
    this.currentImageIndex =
      (this.currentImageIndex - 1 + this.images.length) % this.images.length;
  }
}
