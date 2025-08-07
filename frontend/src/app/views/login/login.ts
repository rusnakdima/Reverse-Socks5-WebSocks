import { Component, signal } from '@angular/core';
import {
  HttpClient,
  HttpErrorResponse,
  HttpHeaders,
} from '@angular/common/http';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';
import { ResponseModel, ResponseStatus } from '@models/response';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [FormsModule, CommonModule],
  templateUrl: './login.html',
})
export class LoginComponent {
  username = signal('');
  password = signal('');
  error = signal('');
  message = signal('');
  token = signal('');

  constructor(
    private http: HttpClient,
    private router: Router,
  ) {}

  login() {
    this.http
      .post<ResponseModel>('http://localhost:7878/api/auth/login', {
        username: this.username(),
        password: this.password(),
      })
      .subscribe({
        next: (response: ResponseModel) => {
          if (response.status == ResponseStatus.Success) {
            this.token.set(response.data);
            this.message.set('Login successful');
            this.error.set('');
            localStorage.setItem('token', response.data);
            setTimeout(() => (window.location.href = '/users/connected'), 2000);
          } else {
            this.error.set(response.message);
            this.message.set('');
          }
        },
        error: (err: HttpErrorResponse) => {
          this.error.set(err.error?.message || 'Invalid credentials');
          this.message.set('');
        },
      });
  }
}
