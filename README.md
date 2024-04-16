# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

- In the Observer pattern diagram explained by the Head First Design Pattern book, Subscriber is defined as an interface. Explain based on your understanding of Observer design patterns, do we still need an interface (or trait in Rust) in this BambangShop case, or a single Model struct is enough?


In the Observer design pattern, the Subscriber typically represents an observer that is interested in being notified of changes in the state of the subject (or publisher). By definition, Different kinds of subscriber objects are interested in the state changes or events of a publisher object, and want to react in their own unique way when the publisher generates an event. Moreover, the publisher wants to
maintain low coupling to the subscribers. For the case of the BambangShop app, it may not necessarily need an interface or trait for the Subscriber if we only have a single type of subscriber with a fixed set of behaviors. If all subscribers behave in the same way and have the same methods (e.g., update()), we can directly use a concrete struct similar with the Subscriber struct (a direct implementation). This is because Rust's ownership system can achieve similar results to Interfaces to ensure loose coupling. Therefore, a single Model struct is enough in this case. However, if we anticipate having different types of subscribers with varying behaviors or if we want to decouple the subject from the concrete implementation of subscribers for flexibility and extensibility, we might consider using an interface (or trait in Rust) for the Subscriber. This would allow us to define a common set of methods that all subscribers must implement, enabling polymorphism and making it easier to add new types of subscribers in the future without modifying the subject.

- id in Program and url in Subscriber is intended to be unique. Explain based on your understanding, is using Vec (list) sufficient or using DashMap (map/dictionary) like we currently use is necessary for this case?


Using a Vec to store your data would require you to manually ensure uniqueness when adding new elements. This would involve iterating through the Vec to check if an element with the same id or url already exists before adding a new one. While possible, this approach is less efficient and prone to errors, especially as the size of your data grows. Given that uniqueness is a fundamental requirement for id in Product and url in Subscriber, using DashMap (or another data structure that enforces uniqueness, such as a HashSet for a single key) is the preferred approach compared to using Vec (list). It provides efficient lookup and insertion operations while ensuring data integrity by enforcing uniqueness constraints automatically since DashMap is a concurrent hashmap implementation in Rust that allows for concurrent read and write access to the map.

- When programming using Rust, we are enforced by rigorous compiler constraints to make a thread-safe program. In the case of the List of Subscribers (SUBSCRIBERS) static variable, we used the DashMap external library for thread safe HashMap. Explain based on your understanding of design patterns, do we still need DashMap or we can implement Singleton pattern instead?

The Singleton pattern ensures that only one instance of a class or struct exists throughout the program's lifetime and also provides a single access point, which can be achieved in Rust through lazy_static or other mechanisms. However, implementing the Singleton pattern directly for managing shared state introduces complexities, such as ensuring safe initialization and access across threads through the use of mutexes. DashMap abstracts away these complexities by providing a ready-to-use thread-safe HashMap implementation since it is designed for concurrent access. Additionally, DashMap is widely used and well-tested, reducing the risk of introducing bugs related to concurrency. Therefore, while implementing a Singleton pattern for managing shared state for the List of Subscribers (SUBSCRIBERS) is possible in Rust, using DashMap or similar thread-safe data structures is often preferred for simplicity, reliability, and maintainability, aligning with Rust's emphasis on safety and performance.


#### Reflection Publisher-2

- In the Model-View Controller (MVC) compound pattern, there is no “Service” and “Repository”. Model in MVC covers both data storage and business logic. Explain based on your understanding of design principles, why we need to separate “Service” and “Repository” from a Model?

Separating "Service" and "Repository" from a Model adheres to the Single Responsibility Principle (SRP). According to SRP, a class or module should have only one reason to change. The "Model" in MVC typically represents the domain logic and data structures of an application. However, including data storage and business logic within the Model can lead to bloated and tightly coupled code, making it harder to maintain and extend. The "Repository" pattern abstracts away data storage and retrieval operations, providing a clean separation between the application's business logic and data access layer. This allows for easier switching between different data storage implementations without affecting the rest of the application. On the other hand, the "Service" layer encapsulates business logic and orchestrates interactions between different parts of the system. Separating business logic into services promotes code reusability, testability, and ensures that each component of the system has a clear responsibility.

- What happens if we only use the Model? Explain your imagination on how the interactions between each model (Program, Subscriber, Notification) affect the code complexity for each model?

If we only use the Model without separating concerns into Service and Repository layers, the code complexity for each model (Product, Subscriber, Notification) would likely increase significantly. Without a clear separation of responsibilities, each model would be burdened with both domain logic and data access operations, leading to bloated and tightly coupled code. For example, the Product model would need to handle not only business logic related to products but also data storage and retrieval operations, such as database queries or file I/O. This would make the Product model less focused and harder to maintain as its responsibilities grow. Similarly, the Subscriber and Notification models would also become more complex if they were responsible for both domain logic and data access. Handling interactions with external systems, such as sending notifications to subscribers, would further increase the complexity of these models.


- Have you explored more about Postman? Tell us how this tool helps you to test your current work. You might want to also list which features in Postman you are interested in or feel like it is helpful to help your Group Project or any of your future software engineering projects.

This tool helps me test my current work since it helped me in testing the endpoints of the program by sending requests, simulate some responses, as well as view the output of each request and response by Postman. For example, it helped me test the output to list all the products, get product by ID, create new product, publish the product by ID, and Delete Product by ID in the Publisher, as well as subscribe/unsubscribe to types of products in the publisher. Conversely, it also helped me test the Receiver by testing the endpoints to view the notification messages and also subscribing/unsubscribing to a certain Type to test if the notification is successfully sent when a CRUD operation is undertaken for a product. Postman may be helpful in my Group Project or other software engineering projects because of its ability to create and organize collections of API requests, making it easy to manage and execute a series of requests in a logical order. This is particularly useful for testing complex workflows or scenarios involving multiple API endpoints. Postman's request builder interface allows for quick and easy creation of various types of HTTP requests (GET, POST, PUT, DELETE, etc.), and its support for different authentication methods, request parameters, headers, and payloads makes it versatile for testing a wide range of APIs. Additionally, Postman's testing capabilities enable the creation of automated tests using JavaScript, allowing for validation of API responses and execution of assertions to ensure that APIs are behaving as expected. This is invaluable for regression testing and ensuring the stability of APIs across different environments. Postman's collaboration features, such as sharing collections and collaborating with team members, also facilitate communication and teamwork in software projects.

#### Reflection Publisher-3

- Observer Pattern has two variations: Push model (publisher pushes data to subscribers) and Pull model (subscribers pull data from publisher). In this tutorial case, which variation of Observer Pattern that we use?

In the provided code, the Observer Pattern is implemented using the Push model, where the publisher (or subject) pushes data to the subscribers (or observers). Specifically, in the NotificationService's notify method, each subscriber's update method is called with the notification payload as an argument. This can be specifically seen in the for-loop within the notify method. The subscribers do not need to actively request data from the publisher; instead, the publisher sends the data to all subscribed observers when an event of interest occurs.

- What are the advantages and disadvantages of using the other variation of Observer Pattern for this tutorial case? (example: if you answer Q1 with Push, then imagine if we used Pull)

In the Pull model variation of the Observer Pattern, subscribers actively request data from the publisher when they need it, rather than the publisher pushing updates to them. This essentially exposes the Subject (Publisher) to the Observer (Subscriber). One advantage of using the Pull model in this case is that it gives subscribers more control over when they receive updates and how frequently they fetch data, which can be beneficial in scenarios where subscribers have varying needs or bandwidth constraints. Additionally, the Pull model can reduce network traffic and overhead since subscribers only request data when they need it, rather than receiving potentially unnecessary updates. However, a drawback of the Pull model is that it can introduce complexity and additional code overhead, as subscribers need to implement logic for requesting and handling data updates. Additionally, there may be latency issues if subscribers don't pull updates frequently enough, potentially leading to outdated information.

- Explain what will happen to the program if we decide to not use multi-threading in the notification process.

If we decide not to use multi-threading in the notification process, the program will likely experience blocking behavior during notification sending. In the current implementation, each notification is sent to subscribers using a separate thread, allowing the program to continue executing without waiting for each notification to be delivered. Without multi-threading, sending notifications to subscribers would occur sequentially, meaning that the program would wait for each notification to be sent before moving on to the next one. This could result in significant delays, especially if there are many subscribers or if sending notifications involves network requests that take a non-trivial amount of time. As a consequence, the overall performance and responsiveness of the program would be negatively impacted. Users may experience delays in response times, and the program's ability to handle concurrent requests or events could be limited. In scenarios where timely notification delivery is critical, such as real-time systems or applications with high concurrency requirements, not using multi-threading for notification processing could lead to suboptimal performance and user experience.

