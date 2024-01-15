insert into users (email, username, password_hash, bio, image) values ('acalderon0@cbsnews.com', 'jcolquitt0', '$2a$04$cHBgM90FUuMPBMD9Upp/meoVuwXE.Kqit7s0gRhjXHOsmPl3fsZMe', 'Quality-focused directional application', 'http://dummyimage.com/179x100.png/cc0000/ffffff');
insert into users (email, username, password_hash, bio, image) values ('cdennitts1@myspace.com', 'gmeechan1', '$2a$04$fFfT0w9CNPHuFxRxlQUXEeyzYlwexIE8xq6W/VBhEibmqpvIpOy5a', 'Mandatory object-oriented support', 'http://dummyimage.com/223x100.png/cc0000/ffffff');
insert into users (email, username, password_hash, bio, image) values ('cblaycock2@macromedia.com', 'cglynne2', '$2a$04$tVzuyHk90XjiURI4BV9PJOwrtPVK6wgugBMA6GwsC5y0g1vW.P8Qm', 'Multi-layered hybrid core', 'http://dummyimage.com/226x100.png/5fa2dd/ffffff');
insert into users (email, username, password_hash, bio, image) values ('wdoutch3@nasa.gov', 'elytle3', '$2a$04$d0LTx9Bz9aSTuELVQTCZhOVhfinJVn7M2roDQDK3Wr4B11R1IaHti', 'Function-based bandwidth-monitored success', 'http://dummyimage.com/246x100.png/5fa2dd/ffffff');
insert into users (email, username, password_hash, bio, image) values ('jdebrett4@samsung.com', 'cvian4', '$2a$04$KELPSGQleEkD05tglj7IqeyOGUOhoRbv.0VQpBLwyl7OlEiyjQKeK', 'Organized impactful artificial intelligence', 'http://dummyimage.com/160x100.png/cc0000/ffffff');
insert into users (email, username, password_hash, bio, image) values ('djuckes5@mozilla.com', 'nbrik5', '$2a$04$7eTY.kH9YPWV5ZI6zgRQRujn0unaZ5Sm7MkQbNb1YshoB/0mkskpe', 'Reduced tangible adapter', 'http://dummyimage.com/244x100.png/5fa2dd/ffffff');
insert into users (email, username, password_hash, bio, image) values ('mpersicke6@fastcompany.com', 'mheatherington6', '$2a$04$Q/8KOV31ytadf6DXRoiXF.zL47h/pAV8l4v2q334QsP7fODESerfO', 'Streamlined fresh-thinking emulation', 'http://dummyimage.com/239x100.png/ff4444/ffffff');
insert into users (email, username, password_hash, bio, image) values ('hsowden7@tmall.com', 'mtowhey7', '$2a$04$/RcRwNMEi8F0q1gXft5S1OflRh9GiidB7nzcHGNMvqBQINgw3SOWe', 'User-centric static architecture', 'http://dummyimage.com/115x100.png/ff4444/ffffff');
insert into users (email, username, password_hash, bio, image) values ('nthonger8@ucoz.ru', 'ainglesfield8', '$2a$04$lH2xiDLEi6axmsGbZndlNe0BtKchuKZ/G4FoQaxQuMiRdkMF0R8GG', 'Adaptive executive success', 'http://dummyimage.com/198x100.png/ff4444/ffffff');
insert into users (email, username, password_hash, bio, image) values ('ssarjent9@histats.com', 'hpobjoy9', '$2a$04$9LYaNYNKbn/2iEpeZDPpyeaCJu73aBQzEZ24aCwogaT7lfUUx0y9C', 'Future-proofed foreground instruction set', 'http://dummyimage.com/156x100.png/ff4444/ffffff');

insert into follows (follower, followee) values (2, 3);
insert into follows (follower, followee) values (2, 8);
insert into follows (follower, followee) values (2, 4);
insert into follows (follower, followee) values (4, 2);
insert into follows (follower, followee) values (4, 5);
insert into follows (follower, followee) values (4, 7);
insert into follows (follower, followee) values (4, 8);
insert into follows (follower, followee) values (5, 6);
insert into follows (follower, followee) values (10, 5);

insert into articles (title, slug, description, body, author_id) values ('vivamus tortor duis mattis egestas metus aenean fermentum donec ut mauris', 'vivamus-tortor-duis-mattis-egestas-metus-aenean-fermentum-donec-ut-mauris', 'orci luctus et ultrices posuere cubilia curae nulla dapibus dolor', 'Donec posuere metus vitae ipsum. Aliquam non mauris. Morbi non lectus. Aliquam sit amet diam in magna bibendum imperdiet. Nullam orci pede, venenatis non, sodales sed, tincidunt eu, felis. Fusce posuere felis sed lacus. Morbi sem mauris, laoreet ut, rhoncus aliquet, pulvinar sed, nisl.', 10);
insert into articles (title, slug, description, body, author_id) values ('sapien sapien non mi integer ac neque duis bibendum morbi non quam nec dui luctus rutrum nulla tellus in', 'sapien-sapien-non-mi-integer-ac-neque-duis-bibendum-morbi-non-quam-nec-dui-luctus-rutrum-nulla-tellus-in', 'sed augue aliquam erat volutpat in congue etiam justo etiam pretium iaculis', 'Curabitur in libero ut massa volutpat convallis. Morbi odio odio, elementum eu, interdum eu, tincidunt in, leo. Maecenas pulvinar lobortis est. Phasellus sit amet erat. Nulla tempus. Vivamus in felis eu sapien cursus vestibulum. Proin eu mi. Nulla ac enim. In tempor, turpis nec euismod scelerisque, quam turpis adipiscing lorem, vitae mattis nibh ligula nec sem. Duis aliquam convallis nunc.', 2);
insert into articles (title, slug, description, body, author_id) values ('vehicula consequat morbi a ipsum integer a nibh in quis justo maecenas', 'vehicula-consequat-morbi-a-ipsum-integer-a-nibh-in-quis-justo-maecenas', 'in faucibus orci luctus et ultrices posuere cubilia curae nulla dapibus', 'In hac habitasse platea dictumst. Maecenas ut massa quis augue luctus tincidunt. Nulla mollis molestie lorem. Quisque ut erat. Curabitur gravida nisi at nibh. In hac habitasse platea dictumst. Aliquam augue quam, sollicitudin vitae, consectetuer eget, rutrum at, lorem. Integer tincidunt ante vel ipsum. Praesent blandit lacinia erat.', 5);
insert into articles (title, slug, description, body, author_id) values ('vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae duis faucibus accumsan odio curabitur', 'vestibulum-ante-ipsum-primis-in-faucibus-orci-luctus-et-ultrices-posuere-cubilia-curae-duis-faucibus-accumsan-odio-curabitur', 'platea dictumst morbi vestibulum velit id pretium iaculis diam erat fermentum', 'Vestibulum sed magna at nunc commodo placerat. Praesent blandit. Nam nulla. Integer pede justo, lacinia eget, tincidunt eget, tempus vel, pede. Morbi porttitor lorem id ligula. Suspendisse ornare consequat lectus. In est risus, auctor sed, tristique in, tempus sit amet, sem. Fusce consequat.', 8);
insert into articles (title, slug, description, body, author_id) values ('aliquet massa id lobortis convallis tortor risus dapibus augue vel accumsan tellus nisi eu orci mauris', 'aliquet-massa-id-lobortis-convallis-tortor-risus-dapibus-augue-vel-accumsan-tellus-nisi-eu-orci-mauris', 'nulla tellus in sagittis dui vel nisl duis ac nibh fusce', 'Proin leo odio, porttitor id, consequat in, consequat ut, nulla. Sed accumsan felis. Ut at dolor quis odio consequat varius. Integer ac leo. Pellentesque ultrices mattis odio. Donec vitae nisi. Nam ultrices, libero non mattis pulvinar, nulla pede ullamcorper augue, a suscipit nulla elit ac nulla. Sed vel enim sit amet nunc viverra dapibus.', 8);
insert into articles (title, slug, description, body, author_id) values ('molestie sed justo pellentesque viverra pede ac diam cras pellentesque volutpat dui maecenas tristique est', 'molestie-sed-justo-pellentesque-viverra-pede-ac-diam-cras-pellentesque-volutpat-dui-maecenas-tristique-est', 'condimentum id luctus nec molestie sed justo pellentesque viverra pede ac diam cras pellentesque', 'Sed ante. Vivamus tortor. Duis mattis egestas metus. Aenean fermentum.', 6);
insert into articles (title, slug, description, body, author_id) values ('sed tincidunt eu felis fusce posuere felis sed lacus morbi sem', 'sed-tincidunt-eu-felis-fusce-posuere-felis-sed-lacus-morbi-sem', 'non ligula pellentesque ultrices phasellus id sapien in sapien iaculis congue vivamus metus arcu adipiscing molestie hendrerit at vulputate', 'Pellentesque viverra pede ac diam. Cras pellentesque volutpat dui. Maecenas tristique, est et tempus semper, est quam pharetra magna, ac consequat metus sapien ut nunc. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Mauris viverra diam vitae quam. Suspendisse potenti. Nullam porttitor lacus at turpis. Donec posuere metus vitae ipsum. Aliquam non mauris. Morbi non lectus. Aliquam sit amet diam in magna bibendum imperdiet.', 6);
insert into articles (title, slug, description, body, author_id) values ('tincidunt nulla mollis molestie lorem quisque ut erat curabitur gravida nisi', 'tincidunt-nulla-mollis-molestie-lorem-quisque-ut-erat-curabitur-gravida-nisi', 'libero rutrum ac lobortis vel dapibus at diam nam tristique', 'Nulla ac enim. In tempor, turpis nec euismod scelerisque, quam turpis adipiscing lorem, vitae mattis nibh ligula nec sem. Duis aliquam convallis nunc. Proin at turpis a pede posuere nonummy. Integer non velit.', 3);
insert into articles (title, slug, description, body, author_id) values ('vehicula condimentum curabitur in libero ut massa volutpat convallis morbi odio odio elementum', 'vehicula-condimentum-curabitur-in-libero-ut-massa-volutpat-convallis-morbi-odio-odio-elementum', 'accumsan tortor quis turpis sed ante vivamus tortor duis mattis', 'Donec semper sapien a libero. Nam dui. Proin leo odio, porttitor id, consequat in, consequat ut, nulla. Sed accumsan felis. Ut at dolor quis odio consequat varius.', 5);
insert into articles (title, slug, description, body, author_id) values ('erat nulla tempus vivamus in felis eu sapien cursus vestibulum proin eu mi nulla ac enim in tempor turpis', 'erat-nulla-tempus-vivamus-in-felis-eu-sapien-cursus-vestibulum-proin-eu-mi-nulla-ac-enim-in-tempor-turpis', 'aliquet ultrices erat tortor sollicitudin mi sit amet lobortis sapien sapien non mi integer', 'Morbi odio odio, elementum eu, interdum eu, tincidunt in, leo. Maecenas pulvinar lobortis est. Phasellus sit amet erat. Nulla tempus. Vivamus in felis eu sapien cursus vestibulum. Proin eu mi.', 10);

insert into tags (tag) values ('ac');
insert into tags (tag) values ('amet');
insert into tags (tag) values ('augue');
insert into tags (tag) values ('condimentum');
insert into tags (tag) values ('cubilia');
insert into tags (tag) values ('elit');
insert into tags (tag) values ('in');
insert into tags (tag) values ('integer');
insert into tags (tag) values ('interdum');
insert into tags (tag) values ('mauris');
insert into tags (tag) values ('metus');
insert into tags (tag) values ('morbi');
insert into tags (tag) values ('mus');
insert into tags (tag) values ('nec');
insert into tags (tag) values ('nulla');
insert into tags (tag) values ('pellentesque');
insert into tags (tag) values ('pretium');
insert into tags (tag) values ('risus');
insert into tags (tag) values ('sapien');
insert into tags (tag) values ('vel');

insert into article_tags (article_id, tag_id) values (10, 10);
insert into article_tags (article_id, tag_id) values (10, 8);
insert into article_tags (article_id, tag_id) values (2, 8);
insert into article_tags (article_id, tag_id) values (3, 16);
insert into article_tags (article_id, tag_id) values (4, 13);
insert into article_tags (article_id, tag_id) values (4, 9);
insert into article_tags (article_id, tag_id) values (5, 17);
insert into article_tags (article_id, tag_id) values (5, 8);
insert into article_tags (article_id, tag_id) values (6, 10);
insert into article_tags (article_id, tag_id) values (6, 16);
insert into article_tags (article_id, tag_id) values (6, 3);
insert into article_tags (article_id, tag_id) values (6, 5);
insert into article_tags (article_id, tag_id) values (6, 7);
insert into article_tags (article_id, tag_id) values (7, 10);
insert into article_tags (article_id, tag_id) values (7, 4);
insert into article_tags (article_id, tag_id) values (7, 6);
insert into article_tags (article_id, tag_id) values (8, 10);
insert into article_tags (article_id, tag_id) values (8, 18);
insert into article_tags (article_id, tag_id) values (9, 11);
insert into article_tags (article_id, tag_id) values (9, 18);

insert into favorites (user_id, article_id) values (1, 1);
insert into favorites (user_id, article_id) values (1, 4);
insert into favorites (user_id, article_id) values (1, 7);
insert into favorites (user_id, article_id) values (1, 8);
insert into favorites (user_id, article_id) values (10, 10);
insert into favorites (user_id, article_id) values (10, 2);
insert into favorites (user_id, article_id) values (2, 3);
insert into favorites (user_id, article_id) values (2, 4);
insert into favorites (user_id, article_id) values (3, 3);
insert into favorites (user_id, article_id) values (4, 6);
insert into favorites (user_id, article_id) values (4, 7);
insert into favorites (user_id, article_id) values (4, 9);
insert into favorites (user_id, article_id) values (5, 4);
insert into favorites (user_id, article_id) values (6, 3);
insert into favorites (user_id, article_id) values (6, 5);
insert into favorites (user_id, article_id) values (6, 6);
insert into favorites (user_id, article_id) values (8, 6);
insert into favorites (user_id, article_id) values (9, 4);

insert into comments (article_id, body, author_id) values (5, 'Nunc purus.', 1);
insert into comments (article_id, body, author_id) values (1, 'Maecenas tristique, est et tempus semper, est quam pharetra magna, ac consequat metus sapien ut nunc.', 7);
insert into comments (article_id, body, author_id) values (4, 'In hac habitasse platea dictumst. Etiam faucibus cursus urna.', 1);
insert into comments (article_id, body, author_id) values (6, 'Nulla mollis molestie lorem. Quisque ut erat. Curabitur gravida nisi at nibh.', 7);
insert into comments (article_id, body, author_id) values (3, 'Cras non velit nec nisi vulputate nonummy.', 10);
insert into comments (article_id, body, author_id) values (7, 'Morbi a ipsum. Integer a nibh. In quis justo.', 3);
insert into comments (article_id, body, author_id) values (9, 'Phasellus sit amet erat. Nulla tempus. Vivamus in felis eu sapien cursus vestibulum.', 4);
insert into comments (article_id, body, author_id) values (8, 'Sed vel enim sit amet nunc viverra dapibus. Nulla suscipit ligula in lacus.', 8);
insert into comments (article_id, body, author_id) values (9, 'Aenean lectus. Pellentesque eget nunc. Donec quis orci eget orci vehicula condimentum.', 1);
insert into comments (article_id, body, author_id) values (6, 'Ut at dolor quis odio consequat varius. Integer ac leo.', 10);
insert into comments (article_id, body, author_id) values (7, 'Phasellus id sapien in sapien iaculis congue.', 9);
insert into comments (article_id, body, author_id) values (6, 'Integer non velit. Donec diam neque, vestibulum eget, vulputate ut, ultrices vel, augue.', 3);
insert into comments (article_id, body, author_id) values (7, 'Suspendisse potenti. In eleifend quam a odio. In hac habitasse platea dictumst.', 1);
insert into comments (article_id, body, author_id) values (3, 'Integer non velit.', 10);
insert into comments (article_id, body, author_id) values (2, 'Vivamus tortor.', 5);
insert into comments (article_id, body, author_id) values (10, 'Curabitur convallis.', 3);
insert into comments (article_id, body, author_id) values (2, 'Praesent blandit lacinia erat. Vestibulum sed magna at nunc commodo placerat. Praesent blandit.', 10);
insert into comments (article_id, body, author_id) values (5, 'Duis bibendum.', 10);
insert into comments (article_id, body, author_id) values (8, 'Nullam orci pede, venenatis non, sodales sed, tincidunt eu, felis. Fusce posuere felis sed lacus.', 6);
insert into comments (article_id, body, author_id) values (6, 'Nulla mollis molestie lorem.', 5);
