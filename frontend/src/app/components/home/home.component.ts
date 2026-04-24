import { Component, OnInit } from '@angular/core';
import { GameService } from '../../services/game.service';
import { Game } from '../../models/game.model';

@Component({
  selector: 'app-home',
  standalone: false,
  templateUrl: './home.component.html',
  styleUrl: './home.component.css'
})
export class HomeComponent implements OnInit {

  genres: string[] = ['Action', 'Adventure', 'RPG', 'Strategy', 'Simulation', 'Sports', 'FPS', 'Platformer'];
  games: Game[] = [];
  searchName: string = '';
  searchGenre: string = '';

  constructor(private gameService: GameService) {}

  ngOnInit() {
    this.gameService.getGames().subscribe({
      next: (data) => {
        this.games = data
      }
    });
  }

  onSearch() {
    this.gameService.getGames(this.searchName, this.searchGenre).subscribe({
      next: (data) => {
        this.games = data
      }
    })
  }

  onReset() {
    this.searchName = ''
    this.searchGenre = ''

    this.gameService.getGames().subscribe(data => {
      this.games = data
    })
  }
}
