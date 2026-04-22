import { Component } from '@angular/core';
import { AuthService } from '../../services/auth.service';
import { FormBuilder, FormGroup } from '@angular/forms';
import { Router } from '@angular/router';

@Component({
  selector: 'app-login',
  standalone: false,
  templateUrl: './login.component.html',
  styleUrl: './login.component.css'
})
export class LoginComponent {

  form: FormGroup

  constructor(private auth: AuthService, private fb: FormBuilder, private router: Router) {
    this.form = this.fb.group({
      username: '',
      password: ''
    });
  }

  

  login() {
    console.log("a")
    this.auth.login(this.form.value).subscribe((res: any) => {
      localStorage.setItem('token', res.token);
      console.log(res.token)
      this.router.navigate(['/']);
    });
  }
}
