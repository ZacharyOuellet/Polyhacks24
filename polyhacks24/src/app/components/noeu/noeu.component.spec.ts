import { ComponentFixture, TestBed } from '@angular/core/testing';

import { NoeuComponent } from './noeu.component';

describe('NoeuComponent', () => {
  let component: NoeuComponent;
  let fixture: ComponentFixture<NoeuComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [NoeuComponent]
    });
    fixture = TestBed.createComponent(NoeuComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
