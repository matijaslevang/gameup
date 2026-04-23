import { HttpClient } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Game } from '../../models/game.model';
import { ImageService } from '../../services/image.service';
import { GameService } from '../../services/game.service';
import { VideoService } from '../../services/video.service';

@Component({
  selector: 'app-game-details',
  standalone: false,
  templateUrl: './game-details.component.html',
  styleUrl: './game-details.component.css'
})
export class GameDetailsComponent implements OnInit {

  game: any;
  images: string[] = [];
  videos: string[] = []

  constructor(
    private route: ActivatedRoute,
    private imageService: ImageService,
    private gameService: GameService,
    private videoService: VideoService
  ) {}

  ngOnInit() {
    const id = this.route.snapshot.paramMap.get('id');

    this.gameService.getGame(id!).subscribe({
      next: (game) => {
        this.game = game
        this.imageService.getImages(id!).subscribe({
          next: (images) => {
            this.images = images
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
}
