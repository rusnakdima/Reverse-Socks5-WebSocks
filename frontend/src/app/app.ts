/* sys lib */
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Component, signal } from '@angular/core';
import { Router, RouterOutlet } from '@angular/router';

/* models */
import { ResponseModel, ResponseStatus } from '@models/response';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.html',
})
export class App {
  constructor(
    private http: HttpClient,
    private router: Router,
  ) {
    this.checkToken();
  }

  protected readonly title = signal('frontend');
  isAuthenticated = signal(false);
  username = signal('');
  isLoading = signal(true);

  checkToken() {
    const token = localStorage.getItem('token');
    if (token) {
      this.http
        .get<ResponseModel>('http://localhost:7878/auth/verify', {
          headers: new HttpHeaders({ Authorization: `Bearer ${token}` }),
        })
        .subscribe({
          next: (response: ResponseModel) => {
            if (response.status === ResponseStatus.Success) {
              this.isAuthenticated.set(true);
              this.username.set(response.data.username || 'User');
            } else {
              this.logout();
            }
            this.isLoading.set(false);
          },
          error: () => {
            this.logout();
            this.isLoading.set(false);
          },
        });
    } else {
      this.logout();
      this.isLoading.set(false);
    }
  }

  logout() {
    localStorage.removeItem('token');
    this.isAuthenticated.set(false);
    this.username.set('');
    this.router.navigate(['/login']);
  }
}
