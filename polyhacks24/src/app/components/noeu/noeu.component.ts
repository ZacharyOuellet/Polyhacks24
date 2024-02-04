import { Component, Input, Output, EventEmitter } from '@angular/core';
import * as d3 from 'd3';

@Component({
  selector: 'app-noeu',
  standalone: true,
  templateUrl: './noeu.component.html',
  styleUrls: ['./noeu.component.scss'],
  template: `<circle [attr.cx]="x" [attr.cy]="y" r="10" fill="red" (click)="onDotClick()"></circle>`,
})
export class NoeuComponent {
  @Input() x: number=0;
  @Input() y: number=0;
  @Input() id: number=0;
  @Output() dotClick: EventEmitter<any> = new EventEmitter<any>();

  constructor(){}
  
  onDotClick(): void {
    this.dotClick.emit();
  }
  click(){
    console.log('id : '+this.id.toString());
  }
}
