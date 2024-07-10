import type { Meta, StoryObj } from '@storybook/vue3';

import DropdownMenu from '../components/ui/dropdown-menu/DropdownMenu.vue';

const meta = {
  title: 'DropdownMenu',
  component: DropdownMenu,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof DropdownMenu>;

export default meta;
type Story = StoryObj<typeof DropdownMenu>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};