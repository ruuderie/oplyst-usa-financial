import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenuRadioItem from '../components/ui/dropdown-menu/DropdownMenuRadioItem.vue';

const meta = {
  title: 'DropdownMenuRadioItem',
  component: DropdownMenuRadioItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenuRadioItem>;

export default meta;
type Story = StoryObj<typeof DropdownMenuRadioItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};