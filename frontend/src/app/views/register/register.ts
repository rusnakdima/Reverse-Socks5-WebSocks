import { Component, signal } from '@angular/core';
import {
  HttpClient,
  HttpHeaders,
  HttpErrorResponse,
} from '@angular/common/http';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';
import { ResponseModel } from '@models/response';

@Component({
  selector: 'app-register',
  standalone: true,
  imports: [FormsModule, CommonModule],
  templateUrl: './register.html',
})
export class RegisterComponent {
  username = signal('');
  password = signal('');
  role = signal('user');
  message = signal('');
  error = signal('');

  constructor(private http: HttpClient, private router: Router) {}

  register() {
    const token = localStorage.getItem('token');
    this.http
      .post<ResponseModel>(
        'http://localhost:7878/auth/register',
        {
          username: this.username(),
          password: this.password(),
          role: this.role(),
        },
        {
          headers: new HttpHeaders({ Authorization: `Bearer ${token}` }),
        }
      )
      .subscribe({
        next: () => {
          this.message.set('User registered successfully');
          this.error.set('');
          setTimeout(() => this.router.navigate(['/users/connected']), 2000);
        },
        error: (err: HttpErrorResponse) => {
          this.error.set(err.error || 'Registration failed');
          this.message.set('');
          if (err.status === 401) {
            this.router.navigate(['/login']);
          }
        },
      });
  }
}
