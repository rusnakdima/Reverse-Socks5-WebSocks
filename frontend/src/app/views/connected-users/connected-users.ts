import { Component, signal } from '@angular/core';
import {
  HttpClient,
  HttpErrorResponse,
  HttpHeaders,
} from '@angular/common/http';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';
import { ResponseModel, ResponseStatus } from '@models/response';

interface Connection {
  username: string;
  ip_address: string;
  connected_at: string;
}

@Component({
  selector: 'app-connected-users',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './connected-users.html',
})
export class ConnectedUsersComponent {
  users = signal<Connection[]>([]);
  error = signal('');

  isAdmin: boolean = false;

  constructor(
    private http: HttpClient,
    private router: Router,
  ) {
    this.loadUsers();
  }

  loadUsers() {
    const token = localStorage.getItem('token');
    this.http
      .get<Connection[]>('http://localhost:7878/connection/list-users', {
        headers: new HttpHeaders({ Authorization: `Bearer ${token}` }),
      })
      .subscribe({
        next: (users) => this.users.set(users),
        error: (err: HttpErrorResponse) => {
          this.error.set(err.error?.message || 'Failed to load users');
          if (err.status === 401) {
            this.router.navigate(['/login']);
          }
        },
      });
  }
}
