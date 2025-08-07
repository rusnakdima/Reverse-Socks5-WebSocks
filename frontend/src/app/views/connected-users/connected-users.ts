import { Component, signal } from '@angular/core';
import {
  HttpClient,
  HttpErrorResponse,
  HttpHeaders,
} from '@angular/common/http';
import { Router } from '@angular/router';
import { CommonModule } from '@angular/common';
import { ResponseModel, ResponseStatus } from '@models/response';
import { MainService } from '@services/main.service';

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
    private mainService: MainService,
    private router: Router,
  ) {
    this.loadUsers();
  }

  loadUsers() {
    this.mainService.loadUsers().subscribe({
      next: (response: ResponseModel) => {
        if (response.status === ResponseStatus.Success) {
          this.users.set(response.data);
        } else {
          this.users.set([
            {
              username: 'Tester',
              ip_address: '127.0.0.1',
              connected_at: new Date().toISOString(),
            },
            {
              username: 'Tester2',
              ip_address: '127.0.0.1',
              connected_at: new Date().toISOString(),
            },
            {
              username: 'Tester3',
              ip_address: '127.0.0.1',
              connected_at: new Date().toISOString(),
            },
            {
              username: 'Tester4',
              ip_address: '127.0.0.1',
              connected_at: new Date().toISOString(),
            },
          ]);
        }
      },
      error: (err: HttpErrorResponse) => {
        this.error.set(err.error?.message || 'Failed to load users');
        if (err.status === 401) {
          this.router.navigate(['/login']);
        }
      },
    });
  }
}
