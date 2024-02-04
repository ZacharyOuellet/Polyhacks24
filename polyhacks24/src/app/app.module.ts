import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import { GraphService } from './service/graph.service';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { HttpManagerService } from './service/http-manager.service';
import { HttpClient, HttpClientModule, HttpHandler } from '@angular/common/http';

@NgModule({
  declarations: [
    AppComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    HttpClientModule
  ],
  providers: [HttpManagerService, GraphService],
  bootstrap: [AppComponent]
})
export class AppModule { }
