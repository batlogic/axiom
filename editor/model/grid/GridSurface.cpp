#include "GridSurface.h"

#include "GridItem.h"
#include "../CollectionViewOperators.h"

using namespace AxiomModel;

GridSurface::GridSurface(AxiomModel::GridSurface::ItemCollection view, QPoint minRect, QPoint maxRect)
    : _grid(minRect, maxRect), _items(std::move(view)), _selectedItems(deriveFunc<GridItem*>(_items, &GridSurface::filterSelected)) {
    _items.itemAdded.connect(this, &GridSurface::handleItemAdded);
}

void GridSurface::doRuntimeUpdate() {
    for (const auto &item : items()) {
        item->doRuntimeUpdate();
    }
}

void GridSurface::saveValue() {
    for (const auto &item : items()) {
        item->saveValue();
    }
}

void GridSurface::restoreValue() {
    for (const auto &item : items()) {
        item->restoreValue();
    }
}

void GridSurface::selectAll() {
    for (const auto &item : items()) {
        item->select(false);
    }
}

void GridSurface::deselectAll() {
    for (const auto &item : items()) {
        item->deselect();
    }
}

void GridSurface::startDragging() {
    lastDragDelta = QPoint(0, 0);
    for (auto &item : selectedItems()) {
        item->startDragging();
    }
}

void GridSurface::dragTo(QPoint delta) {
    if (delta == lastDragDelta) return;

    for (auto &item : selectedItems()) {
        _grid.setRect(item->pos(), item->size(), nullptr);
    }

    auto availableDelta = findAvailableDelta(delta);
    lastDragDelta = availableDelta;
    for (auto &item : selectedItems()) {
        item->dragTo(availableDelta);
    }

    for (auto &item : selectedItems()) {
        _grid.setRect(item->pos(), item->size(), item);
    }
    flushGrid();
}

void GridSurface::finishDragging() {
    for (auto &item : selectedItems()) {
        item->finishDragging();
    }
}

void GridSurface::flushGrid() {
    gridChanged.trigger();
}

std::optional<GridItem*> GridSurface::filterSelected(AxiomModel::GridItem *const &item) {
    return item->isSelected() ? item : std::optional<GridItem*>();
}

bool GridSurface::isAllDragAvailable(QPoint delta) {
    for (auto &item : selectedItems()) {
        if (!item->isDragAvailable(delta)) return false;
    }
    return true;
}

QPoint GridSurface::findAvailableDelta(QPoint delta) {
    if (isAllDragAvailable(delta)) return delta;
    auto xDelta = QPoint(delta.x(), lastDragDelta.y());
    if (isAllDragAvailable(xDelta)) return xDelta;
    auto yDelta = QPoint(lastDragDelta.x(), delta.y());
    if (isAllDragAvailable(yDelta)) return yDelta;
    return lastDragDelta;
}

void GridSurface::handleItemAdded(AxiomModel::GridItem *const &item) {
    item->startedDragging.connect(this, &GridSurface::startDragging);
    item->draggedTo.connect(this, &GridSurface::dragTo);
    item->finishedDragging.connect(this, &GridSurface::finishDragging);
}
