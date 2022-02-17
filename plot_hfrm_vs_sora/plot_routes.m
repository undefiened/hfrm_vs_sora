clear; close all;

pixels_per_km = 131/2;
km_per_pixel = 1/pixels_per_km;
m_per_pixel = km_per_pixel*1000;

population_multiplier = 0.00865526;

sheltered_people = 1;
probability_of_killing = 1;

number_of_event_per_hour = 10^-4;

r_m = 150;
speed_m_s = 10; % 36 km/h
drone_area_m_2 = 3.87;

aircraft_people = 180;

reverse_y = true;


routes = jsondecode(fileread(['./results/res_nk.json']));
map = imread('./data/density_fixed_scaled.png');
% map = jsondecode(fileread(['./data/exposure_map' postfix '.json']));

airrisk = jsondecode(fileread('./data/map.json'));

%% Plot ground risk vs length

data = [];
data_per_hour = [];

for i = 1:size(routes, 1)
    area = routes(i).length_m * r_m * 2;
    density = (routes(i).ground_risk * population_multiplier)/ area;
    efr_per_hour = number_of_event_per_hour * drone_area_m_2 * density * sheltered_people * probability_of_killing;
    
    efr = ((routes(i).length_m / speed_m_s)/3600) * efr_per_hour;
    
    data = [data; routes(i).length_m efr];
    data_per_hour = [data_per_hour; routes(i).length_m efr_per_hour];
end

% figure;
% 
% scatter(data(:, 1), data(:, 2));
% 
% xlabel('Length (m)');
% ylabel('EFR (ppl)');
% 
% saveas(gcf, './pics/nk_ground_risks.png');

figure;
hold on;

for i=1:size(data_per_hour, 1)
    l = plot(data_per_hour(i, 1), data_per_hour(i, 2), 'o');
    l.MarkerFaceColor = l.Color;
end

xlabel('Length, m');
ylabel('EFR, hr^{-1} ');

saveas(gcf, './pics/nk_ground_risks.png');

%% Plot air risk

% 
% data = [];
% 
% for i = 1:size(routes, 1)
%     data = [data; routes(i).length_m routes(i).air_risk * aircraft_people/2];
% end
% 
% figure;
% 
% scatter(data(:, 1), data(:, 2));
% 
% xlabel('Length, m');
% ylabel('EFR');
% 
% saveas(gcf, './pics/nk_air_risks.png');



%% Plot airrisk map
% figure;
% imagesc(airrisk');
% colorbar;

%% Plot map

figure;
hold on;

imagesc(map);
% imagesc(airrisk');

legends = {};
for route=1:length(routes)
    coords = [routes(route).route.x; routes(route).route.y]';

    plot(coords(:, 1), coords(:, 2), 'LineWidth', 4);
    legends{route} = num2str(routes(route).alpha);
end

if reverse_y
    set(gca, 'YDir','reverse');
end

plot([coords(1, 1), ], [coords(1, 2), ], '.', 'MarkerSize', 80, 'Color', 'm');
plot([coords(end, 1), ], [coords(end, 2), ], '.', 'MarkerSize', 80, 'Color', 'b');

axis equal;

% xlim([1, size(map, 2)])
% ylim([1, size(map, 1)])
xlim([379.1773  923.1249]);
ylim([284.7060  770.3401])

set(gca,'xtick',[]);
set(gca,'xticklabel',[]);
set(gca,'ytick',[]);
set(gca,'yticklabel',[]);

axis off
exportgraphics(gcf, './pics/nk_routes.png');
% saveas(gcf, './pics/nk_routes.png');