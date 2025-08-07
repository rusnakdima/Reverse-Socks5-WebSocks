/* sys lib */
import {
  HttpClient,
  HttpErrorResponse,
  HttpHeaders,
} from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

/* models */
import { ResponseModel, ResponseStatus } from '@models/response';

@Injectable({
  providedIn: 'root',
})
export class MainService {
  token: string = '';
  constructor(private http: HttpClient) {
    this.token = localStorage.getItem('token') ?? '';
  }

  checkToken(): Observable<ResponseModel> {
    return this.http.get<ResponseModel>(
      'http://localhost:7878/api/auth/verify',
      {
        headers: new HttpHeaders({ Authorization: `Bearer ${this.token}` }),
      },
    );
  }

  startConnection(): Observable<ResponseModel> {
    return this.http.get<ResponseModel>(
      'http://localhost:7878/api/connection/start',
      {
        headers: new HttpHeaders({ Authorization: `Bearer ${this.token}` }),
      },
    );
  }
  login(username: string, password: string): Observable<ResponseModel> {
    return this.http.post<ResponseModel>(
      'http://localhost:7878/api/auth/login',
      {
        username,
        password,
      },
    );
  }

  register(
    username: string,
    password: string,
    role: string,
  ): Observable<ResponseModel> {
    return this.http.post<ResponseModel>(
      'http://localhost:7878/api/auth/register',
      {
        username,
        password,
        role,
      },
      {
        headers: new HttpHeaders({ Authorization: `Bearer ${this.token}` }),
      },
    );
  }

  loadUsers(): Observable<ResponseModel> {
    return this.http.get<ResponseModel>(
      'http://localhost:7878/api/connection/list-users',
      {
        headers: new HttpHeaders({ Authorization: `Bearer ${this.token}` }),
      },
    );
  }
}
