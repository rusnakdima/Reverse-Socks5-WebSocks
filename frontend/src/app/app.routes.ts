/* sys lib */
import { Routes } from '@angular/router';

/* guard */
import { AuthGuard } from './guards/auth.guard';

/* component */
import { LoginComponent } from '@views/login/login';
import { ConnectedUsersComponent } from '@views/connected-users/connected-users';
import { RegisterComponent } from '@views/register/register';

export const routes: Routes = [
  { path: '', redirectTo: 'login', pathMatch: 'full' },
  { path: 'login', component: LoginComponent },
  {
    path: 'users/connected',
    component: ConnectedUsersComponent,
    canActivate: [AuthGuard],
  },
  { path: 'register', component: RegisterComponent, canActivate: [AuthGuard] },
];
