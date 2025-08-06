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

  constructor(
    private http: HttpClient,
    private router: Router,
  ) {}

  login() {
    this.http
      .post<ResponseModel>('http://localhost:7878/auth/login', {
        username: this.username(),
        password: this.password(),
      })
      .subscribe({
        next: (response: ResponseModel) => {
          if (response.status == ResponseStatus.Success) {
            localStorage.setItem('token', response.data);
            this.startConnection(response.data);
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

  startConnection(token: string) {
    this.http
      .get<ResponseModel>('http://localhost:7878/connection/start', {
        headers: new HttpHeaders({ Authorization: `Bearer ${token}` }),
      })
      .subscribe({
        next: (response: ResponseModel) => {
          console.log(response);
          if (response.status == ResponseStatus.Success) {
            this.message.set('Connection initiated successfully');
            this.error.set('');
            setTimeout(() => (window.location.href = '/users/connected'), 2000);
          } else {
            this.error.set(response.message);
            this.message.set('');
          }
        },
        error: (err: HttpErrorResponse) => {
          console.log(err);
          this.error.set(err.error?.text || 'Failed to initiate connection');
          this.message.set('');
          if (err.status === 401) {
            this.router.navigate(['/login']);
          }
        },
      });
  }
}
