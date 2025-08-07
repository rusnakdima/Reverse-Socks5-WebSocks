/* sys lib */
import { HttpErrorResponse } from '@angular/common/http';
import { Component, signal } from '@angular/core';
import { Router, RouterModule, RouterOutlet } from '@angular/router';

/* models */
import { ResponseModel, ResponseStatus } from '@models/response';

/* services */
import { MainService } from '@services/main.service';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, RouterModule],
  templateUrl: './app.html',
})
export class App {
  constructor(
    private mainService: MainService,
    private router: Router,
  ) {
    this.checkToken();
  }

  protected readonly title = signal('frontend');
  isAuthenticated = signal(false);
  username = signal('');
  isLoading = signal(true);
  isAdmin: boolean = false;

  checkToken() {
    this.mainService.checkToken().subscribe({
      next: (response: ResponseModel) => {
        if (response.status === ResponseStatus.Success) {
          this.isAuthenticated.set(true);
          this.username.set(response.data.username || 'User');
          this.isAdmin = response.data.role === 'admin';
          this.startConnection();
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
  }

  startConnection() {
    this.mainService.startConnection().subscribe({
      next: (response: ResponseModel) => {
        if (response.status == ResponseStatus.Error) {
          console.error(response.message);
        }
      },
      error: (err: HttpErrorResponse) => {
        console.log(err);
      },
    });
  }

  logout() {
    localStorage.removeItem('token');
    this.isAuthenticated.set(false);
    this.username.set('');
    this.router.navigate(['/login']);
  }
}
