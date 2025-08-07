/* sys lib */
import { Component, signal } from '@angular/core';
import { HttpErrorResponse } from '@angular/common/http';
import { FormsModule } from '@angular/forms';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';

/* services */
import { MainService } from '@services/main.service';

@Component({
  selector: 'app-register',
  standalone: true,
  providers: [MainService],
  imports: [FormsModule, CommonModule],
  templateUrl: './register.html',
})
export class RegisterComponent {
  username = signal('');
  password = signal('');
  role = signal('user');
  message = signal('');
  error = signal('');

  constructor(
    private mainService: MainService,
    private router: Router,
  ) {}

  register() {
    const token = localStorage.getItem('token');
    this.mainService
      .register(this.username(), this.password(), this.role())
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
