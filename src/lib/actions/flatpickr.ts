import flatpickr from 'flatpickr';
import type { Instance } from 'flatpickr/dist/types/instance';
import type { Options } from 'flatpickr/dist/types/options';

interface FlatpickrParams {
    defaultDate?: string | Date;
    onChange?: (dateStr: string, selectedDates: Date[]) => void;
    options?: Partial<Options>;
}

export function flatpickrAction(node: HTMLInputElement, params: FlatpickrParams = {}) {
    let fp: Instance;

    function create(p: FlatpickrParams) {
        fp = flatpickr(node, {
            enableTime: true,
            time_24hr: true,
            dateFormat: 'Y-m-d H:i',
            defaultDate: p.defaultDate,
            ...p.options,
            onChange: (selectedDates, dateStr) => {
                p.onChange?.(dateStr, selectedDates);
            }
        });
    }

    create(params);

    return {
        update(newParams: FlatpickrParams) {
            fp.destroy();
            create(newParams);
        },
        destroy() {
            fp.destroy();
        }
    };
}