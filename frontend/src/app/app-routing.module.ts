import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { HomeComponent } from './components/home/home.component';
import { GameDetailsComponent } from './components/game-details/game-details.component';
import { LoginComponent } from './components/login/login.component';
import { CreateGameComponent } from './components/create-game/create-game.component';
import { AuthGuard } from './guards/auth.guard';
import { EditGameComponent } from './components/edit-game/edit-game.component';

const routes: Routes = [
  { path: '', component: HomeComponent },
  { path: 'games/:id', component: GameDetailsComponent },
  { path: 'login', component: LoginComponent },
  { path: 'add-game', component: CreateGameComponent, canActivate: [AuthGuard]},
  { path: 'games/edit/:id', component: EditGameComponent, canActivate: [AuthGuard]}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {}