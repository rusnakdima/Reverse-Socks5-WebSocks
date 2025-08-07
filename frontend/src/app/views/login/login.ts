/* sys lib */
import { Component, signal } from '@angular/core';
import { HttpErrorResponse } from '@angular/common/http';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';

/* models */
import { ResponseModel, ResponseStatus } from '@models/response';

/* services */
import { MainService } from '@services/main.service';

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
    private mainService: MainService,
    private router: Router,
  ) {}

  login() {
    this.mainService.login(this.username(), this.password()).subscribe({
      next: (response: ResponseModel) => {
        if (response.status == ResponseStatus.Success) {
          this.token.set(response.data);
          this.message.set('Login successful');
          this.error.set('');
          localStorage.setItem('token', response.data);
          window.location.href = '/users/connected';
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
