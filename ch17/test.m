clc; clear;

areax = [20:30];
areay = [-10:-5];

count = 50;
pos = [0, 0];
vel = [17, -4];
stepX = [pos(1)];
stepY = [pos(2)];

for i=0:count
  pos(1) += vel(1);
  pos(2) += vel(2);
  
  vel(1) = max(vel(1) - sign(vel(1)), 0);
  vel(2) -= 1;
  
  stepX = [stepX, pos(1)];
  stepY = [stepY, pos(2)];
endfor

for y=areay
  for x=areax
    hold on;
    plot(x, y, "o");
  endfor
endfor

hold on;
plot(stepX, stepY, "go");
