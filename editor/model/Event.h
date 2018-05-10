#pragma once

#include <vector>
#include <optional>
#include <functional>
#include <set>
#include <utility>

#include "Hookable.h"

namespace AxiomModel {

    template<class... Args>
    class Event : public Hookable {
    private:
        using owned_collection = typename std::vector<Event>;

    public:
        using func_type = std::function<void(Args...)>;

        Event() noexcept = default;

        explicit Event(func_type func) noexcept : callback(func) {}

        Event(const Event &a) noexcept : callback(a.callback) {
            copyFrom(a);
        }

        Event(Event &&a) noexcept {
            moveFrom(a);
        }

        Event &operator=(const Event &a) noexcept {
            detachAll();
            copyFrom(a);
            return *this;
        }

        Event &operator=(Event &&a) noexcept {
            detachAll();
            moveFrom(a);
            return *this;
        }

        ~Event() override {
            detachAll();
        }

        void trigger(Args... params) const {
            if (callback) {
                (*callback)(params...);
            }

            for (const auto &listener : listeners) {
                listener->trigger(params...);
            }
        }

        Event *listen(Event listener) {
            ownedListeners.push_back(std::move(listener));
            auto ptr = &ownedListeners.back();
            connect(ptr);
            return ptr;
        }

        Event *listen(func_type listener) {
            return listen(Event(listener));
        }

        Event *listen(Hookable *follow, Event listener) {
            auto result = listen(std::move(listener));
            result->follow(follow);
            return result;
        }

        template<class TR, class... TA>
        Event *listen(Hookable *follow, std::function<TR(TA...)> listener) {
            return listen(follow, Event(std::function([listener](Args&&... params) {
                applyFunc<sizeof...(TA)>(listener, std::forward<Args>(params)...);
            })));
        }

        template<class TB, class TFB, class TR, class... TA>
        Event *listen(TB *follow, TR (TFB::*listener)(TA...)) {
            auto wrapper = std::mem_fn(listener);
            return listen(follow, Event(std::function([follow, wrapper](Args&&... params) {
                applyFunc<sizeof...(TA) + 1>(wrapper, follow, std::forward<Args>(params)...);
            })));
        }

        template<class TB, class TFB, class TR, class... TA>
        Event *forward(TB *handler, TR (TFB::*listener)(TA...)) {
            auto wrapper = std::mem_fn(listener);
            return listen(Event(std::function([handler, wrapper](Args&&... params) {
                applyFunc<sizeof...(TA) + 1>(wrapper, handler, std::forward<Args>(params)...);
            })));
        }

        void connect(Event *other) {
            listeners.emplace(other);
            other->invListeners.emplace(this);
        }

        void disconnect(Event *other) {
            listeners.erase(other);
            other->invListeners.erase(this);

            auto iter = typename owned_collection::iterator(other);
            if (iter >= ownedListeners.begin() && iter < ownedListeners.end()) {
                ownedListeners.erase(iter);
            }
        }

        void follow(Hookable *other) {
            following.emplace(other);
            other->addDestructHook(this, [this]() { detachAll(); });
        }

        void unfollow(Hookable *other) {
            following.erase(other);
            other->removeDestructHook(this);
        }

    private:
        std::optional<func_type> callback;
        owned_collection ownedListeners;
        std::set<Event *> listeners;
        std::set<Event *> invListeners;
        std::set<Hookable *> following;

        void detachAll() {
            while (!invListeners.empty()) {
                (*invListeners.begin())->disconnect(this);
            }
            while (!following.empty()) {
                unfollow(*following.begin());
            }
        }

        void copyFrom(const Event &a) {
            for (const auto &listener : a.listeners) {
                auto iter = typename owned_collection::iterator(listener);
                if (iter >= a.ownedListeners.begin() && iter < a.ownedListeners.end()) {
                    listen(*iter);
                } else {
                    connect(listener);
                }
            }

            for (const auto &evt : a.following) {
                follow(evt);
            }
        }

        void moveFrom(Event &a) {
            callback = std::move(a.callback);
            ownedListeners = std::move(a.ownedListeners);
            listeners = std::move(a.listeners);
            invListeners = std::move(a.invListeners);
            following = std::move(a.following);

            for (const auto &listener : listeners) {
                listener->invListeners.erase(&a);
                listener->invListeners.emplace(this);
            }
            for (const auto &hook : following) {
                hook->removeDestructHook(&a);
                hook->addDestructHook(this, [this]() { detachAll(); });
            }
        }

        template<class Func, size_t... I, class... PassArgs>
        static void applyFuncIndexed(const Func &func, std::index_sequence<I...>, PassArgs&&... params) {
            func(std::get<I>(std::make_tuple(std::forward<PassArgs>(params)...))...);
        }

        template<size_t ArgCount, class Func, class... PassArgs>
        static void applyFunc(const Func &func, PassArgs&&... params) {
            applyFuncIndexed(func, std::make_index_sequence<ArgCount>{}, std::forward<PassArgs>(params)...);
        }
    };

}
